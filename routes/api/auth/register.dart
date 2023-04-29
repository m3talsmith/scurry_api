import 'dart:convert';
import 'dart:io';

import 'package:dart_frog/dart_frog.dart';

import 'package:scurry_api/user/user.dart';

Future<Response> onRequest(RequestContext context) async {
  final req = context.request;
  if (req.method != HttpMethod.post) {
    return Response(
      statusCode: HttpStatus.notFound,
      body: 'route not found',
    );
  }
  final body = await req.body();
  if (body.isEmpty) {
    return Response(
      body: 'request missing body',
      statusCode: HttpStatus.badRequest,
    );
  }
  User user = User.fromJson(jsonDecode(body) as Map<String, dynamic>);
  await user.create();
  return Response(body: jsonEncode(user));
}
